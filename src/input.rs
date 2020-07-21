
pub struct Input<'a> {
    data : &'a [(usize, char)] 
}

#[derive(Clone, Copy)]
pub struct RestorePoint<'a> {
    data : &'a [(usize, char)] 
}

#[derive(Debug)]
pub struct PSym {
    pub value : String,
    pub start : usize,
    pub end : usize,
}

#[derive(Debug)]
pub enum InputError { 
    EndOfFileInComment 
}

impl<'a> Input<'a> {

    pub fn new(input : &'a [(usize, char)] ) -> Input<'a> { 
        Input { data: input }
    }

    pub fn create_restore(&self) -> RestorePoint<'a> {
        RestorePoint{ data: self.data }
    }

    pub fn restore(&mut self, restore_point : RestorePoint<'a>) {
        self.data = restore_point.data 
    }

    pub fn parse_symbol(&mut self) -> Result<PSym, InputError> {
        self.clear()?;
         
        let mut d = self.data;
        let mut cs = vec![];
        let start : usize;
        let mut end = 0;

        loop {
            match d {
                [] => break,
                [(i, x), rest @ ..] if !x.is_whitespace() => {
                    d = rest;
                    cs.push(x);
                    end = *i;
                },
                [_, ..] => break,
            }
        }

        self.data = d;

        Ok( PSym { start, end, value: cs.into_iter().collect::<String>() } )
    }

    pub fn clear(&mut self) -> Result<(), ParseError> { 
        let mut d = self.data;
        let mut comment = 0;
        loop {
            match d {
                [] if comment > 0 => return Err(InputError::EndOfFileInComment)
                [] => break,
                [(_, '/'), (_, '*'), rest @ ..] => {
                    comment += 1;
                    d = rest; 
                },
                [(_, '*'), (_, '/'), rest @ ..] if comment > 0 => {
                    comment -= 1;
                    d = rest; 
                }, 
                [_, rest @ ..] if comment > 0 => d = rest,
                [(_, x), rest @ ..] if x.is_whitespace() => d = rest,
                _ => break,
            }
        }
        self.data = d;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*#[test]
    fn should_expect_string() -> Result<(), ParseError> {
        let mut input = Input { data: &"::<>::".char_indices().collect::<Vec<(usize, char)>>() };
        input.expect("::<>::")?;
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), "".to_string() ); 
        Ok(())
    }*/
}
