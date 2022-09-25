use super::*;

use ::tokio_util::codec::Decoder;

use ::amq_protocol::frame::parse_frame;

impl Decoder for AmqpFrameCodec {
    type Item = AMQPFrame;
    type Error = DecodeFailure;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let ctx = ParsingContext::wrap(&src[..]);

        match parse_frame(ctx) {
            Ok((ctx_next, frame)) => {
                log::trace!("frame: {:?}", frame);

                let split_to = ctx_next.start;
                let _ = src.split_to(split_to);

                Ok(Some(frame))
            },
            Err(nom_err) => match nom_err {
                nom::Err::Incomplete(needed) => {
                    log::trace!("incomplete: {:?}", needed);
                    Ok(None)
                },
                nom::Err::Failure(err) => Err(DecodeFailure::ParseError(err)),
                nom::Err::Error(err) => Err(DecodeFailure::ParseError(err)),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ParsingContext<'a> {
    slice: &'a [u8],
    start: usize,
    end: usize,
}

impl<'a> ParsingContext<'a> {
    pub fn new(slice: &'a [u8], start: usize, end: usize) -> Self {
        Self { slice, start, end }
    }

    pub fn wrap(slice: &'a [u8]) -> Self {
        Self::new(slice, 0, slice.len())
    }

    pub fn iter(&self) -> std::slice::Iter<'a, u8> {
        self.slice[self.start..self.end].iter()
    }

    fn sub(&self, start: usize, end: usize) -> Self {
        Self::new(self.slice, self.start + start, self.start + end)
    }
}

mod impl_buf_traits {
    use super::*;

    use ::nom::{InputIter, InputLength, InputTake, Slice, UnspecializedInput};
    use std::iter::{Cloned, Enumerate};
    use std::ops::RangeFrom;
    use std::slice::Iter;

    impl<'a> InputLength for ParsingContext<'a> {
        fn input_len(&self) -> usize {
            self.end - self.start
        }
    }

    impl<'a> InputIter for ParsingContext<'a> {
        /// the current input type is a sequence of that `Item` type.
        ///
        /// example: `u8` for `&[u8]` or `char` for &str`
        type Item = u8;
        /// an iterator over the input type, producing the item and its position
        /// for use with [Slice]. If we're iterating over `&str`, the position
        /// corresponds to the byte index of the character
        type Iter = Enumerate<Self::IterElem>;

        /// an iterator over the input type, producing the item
        type IterElem = Cloned<Iter<'a, u8>>;

        /// returns an iterator over the elements and their byte offsets
        fn iter_indices(&self) -> Self::Iter {
            self.iter_elements().enumerate()
        }
        /// returns an iterator over the elements
        fn iter_elements(&self) -> Self::IterElem {
            self.iter().cloned()
        }
        /// finds the byte position of the element
        fn position<P>(&self, predicate: P) -> Option<usize>
        where
            P: Fn(Self::Item) -> bool,
        {
            self.iter().position(|b| predicate(*b))
        }
        /// get the byte offset from the element's position in the stream
        fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
            if self.input_len() >= count {
                Ok(count)
            } else {
                Err(nom::Needed::Unknown)
            }
        }
    }

    impl<'a> InputTake for ParsingContext<'a> {
        /// returns a slice of `count` bytes. panics if count > length
        fn take(&self, count: usize) -> Self {
            self.sub(0, count)
        }

        /// split the stream at the `count` byte offset. panics if count > length
        fn take_split(&self, count: usize) -> (Self, Self) {
            (self.sub(count, self.input_len()), self.sub(0, count))
        }
    }

    impl<'a> Slice<RangeFrom<usize>> for ParsingContext<'a> {
        fn slice(&self, range: RangeFrom<usize>) -> Self {
            self.sub(range.start, self.input_len())
        }
    }

    impl<'a> UnspecializedInput for ParsingContext<'a> {}
}
