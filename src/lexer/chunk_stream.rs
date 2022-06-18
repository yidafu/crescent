
pub(crate) struct ChunkStream {
    pub chunk: Vec<char>,
    pub chunk_name: String,
    pub line: i32,
    pub column: i32,
    pub index: usize,
}


impl ChunkStream {
    pub fn new(chunk_name: &str, chunk: &str) -> ChunkStream {
      ChunkStream {
        chunk_name: String::from(chunk_name),
        chunk: String::from(chunk).chars().collect(),
        line: 1,
        column: 0,
        index: 0
      }
    }

    pub fn next(&mut self) -> char {
        if self.eof() {
          return '\0';
        }
        let char = self.chunk[self.index];
        self.index += 1;
        if char == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        char
    }

    pub fn peek(&self) -> char {
      if self.eof() {
        '\0'
      } else {
        self.chunk[self.index]
      }
    }

    pub fn peek2(&self) -> char {
      if self.eof() {
        '\0'
      } else {
        self.chunk[self.index + 1]
      }
    }

    pub fn eof(&self) -> bool {
        self.index >= self.chunk.len()
    }

    pub fn get_position(&self) -> String {
        format!("line: {:+}, cloumn: {:+}", self.line, self.column)
    }
}

#[test]
fn test_chuck_steam() {
  let mut chunk_stream = ChunkStream::new("test.lua", "line1\nline2");

  assert_eq!(chunk_stream.next(), 'l');
  assert_eq!(chunk_stream.line, 1);
  assert_eq!(chunk_stream.column, 1);
  assert_eq!(chunk_stream.index, 1);
  chunk_stream.next(); // eat i
  chunk_stream.next(); // eat n
  chunk_stream.next(); // eat e
  chunk_stream.next(); // eat 1
  chunk_stream.next(); // eat \n

  assert_eq!(chunk_stream.next(), 'l');
  assert_eq!(chunk_stream.line, 2);
  assert_eq!(chunk_stream.column, 1);
  assert_eq!(chunk_stream.index, 7);
  assert_eq!(chunk_stream.eof(), false);

  chunk_stream.next(); // eat i
  chunk_stream.next(); // eat n
  chunk_stream.next(); // eat e
  chunk_stream.next(); // eat 2

  assert_eq!(chunk_stream.next(), '\0');
  assert_eq!(chunk_stream.eof(), true);
}