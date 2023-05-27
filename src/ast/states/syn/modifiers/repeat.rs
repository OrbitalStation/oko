use crate::*;
use core::marker::PhantomData;

pub struct IgnoringParseRepeatedly <T: Parseable, const AT_LEAST: usize> (PhantomData <T>);

impl <T: Parseable, const AT_LEAST: usize> Parseable for IgnoringParseRepeatedly <T, AT_LEAST> {
    fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
        let mut already_done = 0;
        loop {
            match parser.parse::<T>() {
                Ok(_) => (),
                Err(x) => return if already_done >= AT_LEAST {
                    Ok(Self(PhantomData))
                } else {
                    Err(x)
                }
            };
            already_done += 1
        }
    }

    fn add_to_span(&mut self, _: usize) {}
}

#[derive(Debug, Clone, Visitable)]
pub struct ParseRepeatedly <T, Sep, const AT_LEAST: usize> {
    pub vec: Vec <T>,
    #[novisit]
    pub trailing_sep: bool,
    #[novisit]
    pub last_error: Box <ParseError>,
    #[novisit]
    pub _marker: PhantomData <Sep>
}

impl <T: Parseable, Sep: Parseable, const AT_LEAST: usize> Parseable for ParseRepeatedly <T, Sep, AT_LEAST> {
    fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
        let mut already_done = 0;
        let mut vec = vec![];
        loop {
            match parser.parse::<T>() {
                Ok(x) => vec.push(x),
                Err(x) => return if already_done >= AT_LEAST {
                    Ok(Self {
                        trailing_sep: !vec.is_empty(),
                        vec,
                        last_error: Box::new(x),
                        _marker: PhantomData
                    })
                } else {
                    Err(x)
                }
            };
            match parser.parse::<Sep>() {
                Ok(_) => (),
                Err(x) => return if already_done >= AT_LEAST {
                    Ok(Self {
                        vec,
                        trailing_sep: false,
                        last_error: Box::new(x),
                        _marker: PhantomData
                    })
                } else {
                    Err(x)
                }
            }
            already_done += 1
        }
    }

    fn add_to_span(&mut self, offset: usize) {
        for x in &mut self.vec {
            x.add_to_span(offset)
        }
    }
}
