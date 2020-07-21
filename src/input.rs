
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
    EndOfFileInComment,
    EndOfFileInSymbol,
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

    pub fn more(&mut self) -> bool {
        match self.data {
            [] => false,
            _ => true,
        }
    }

    pub fn parse_symbol(&mut self) -> Result<PSym, InputError> {
        self.clear()?;
         
        let mut d = self.data;
        let mut cs = vec![];
        let start : usize;
        let mut end = 0;

        match d {
            [] => return Err(InputError::EndOfFileInSymbol),
            [(i,c), rest @ ..] if *c == '"' => {
                self.data = rest; 
                return Ok(PSym { start: *i, end: *i, value: "\"".to_string() });
            },
            [(i,_), ..] => start = *i,
        }

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

    pub fn clear(&mut self) -> Result<(), InputError> { 
        let mut d = self.data;
        let mut comment = 0;
        loop {
            match d {
                [] if comment > 0 => return Err(InputError::EndOfFileInComment),
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

    #[test]
    fn should_parse_symbols() -> Result<(), InputError> {
        let mut input = Input { data: &r#"sym 123 , /* /* */ */ <>"#.char_indices().collect::<Vec<(usize, char)>>() };
        let mut symbols = vec![];
        while input.more() {
            let sym = input.parse_symbol()?; 
            symbols.push(sym);
        }
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), "".to_string() ); 
        assert_eq!( symbols.len(), 4 );
        assert_eq!( symbols[0].value, "sym" );
        assert_eq!( symbols[1].value, "123" );
        assert_eq!( symbols[2].value, "," );
        assert_eq!( symbols[3].value, "<>" );
        Ok(())
    }

    #[test]
    fn should_parse_symbols_with_double_quote() -> Result<(), InputError> {
        let mut input = Input { data: &r#"sym " 123 , /* /* */ */ <>"#.char_indices().collect::<Vec<(usize, char)>>() };
        let mut symbols = vec![];
        while input.more() {
            let sym = input.parse_symbol()?; 
            symbols.push(sym);
        }
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), "".to_string() ); 
        assert_eq!( symbols.len(), 5 );
        assert_eq!( symbols[0].value, "sym" );
        assert_eq!( symbols[1].value, "\"" );
        assert_eq!( symbols[2].value, "123" );
        assert_eq!( symbols[3].value, "," );
        assert_eq!( symbols[4].value, "<>" );
        Ok(())
    }
}
