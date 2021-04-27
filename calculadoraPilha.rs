use std::io;
use std::str::FromStr;
use std::process;

#[derive(Debug)]
struct Pilha<T> {
  pilha: Vec<T>
}

impl<T> Pilha<T> {
  fn new() -> Self {
    Pilha { pilha: Vec::new() }
  }

  fn length(&self) -> usize {
    self.pilha.len()
  }

  fn push(&mut self, item: T) {
    self.pilha.push(item)
  }

  fn pop(&mut self) -> Option<T> {
    self.pilha.pop()
  }
  
  fn is_empty(&self) -> bool {
    self.pilha.is_empty()
  }

  fn peek(&self) -> Option<&T> {
    self.pilha.first()
  }
}

fn inserir(pilha: &mut Pilha<i32>){
  let mut inputLine: String = String::new();
  
  loop{
    println!("Enter a number or mathematical operation or write 'quit' to quit the calculator ");
    
    inputLine.clear();
    io::stdin().read_line(&mut inputLine).unwrap();

    if inputLine.trim() == "quit" {
      println!("You quit the calculator! ");
      process::exit(0);
      break;
    }

    if pilha.length() == 100{
      println!("Please restart the calculator ");
      break;
    }

    match &inputLine.trim() as &str {
      "*" | "/" | "+" | "-" | "%" => {
        if pilha.length() < 2{
          println!(" ");
          println!("To do any mathematical operation you need 2 numbers and for now the list does not contain what is needed, so write another number and redo the operation");
          println!(" ");
          continue;
        }
        let number1 = pilha.pop().unwrap();
        let number2 = pilha.pop().unwrap();
        pilha.push(operator(&inputLine, number1, number2));
        
      },
      _ => {
        let number: i32 = i32::from_str(inputLine.trim()).unwrap_or(0);
        pilha.push(number)
      },
    }    
  }
}

fn operator(operator: &String, number1: i32, number2: i32) -> i32 {
  let resultOperation: i32 = match &operator.trim() as &str {
    "*" => number2 * number1,
    "/" => number2 / number1,
    "+" => number2 + number1,
    "-" => number2 - number1,
    "%" => number2 % number1,
    _ => 0,
  };

  resultOperation
}

fn main() {
  let mut pilha: Pilha<i32> = Pilha::<i32>::new();
  inserir(&mut pilha);
  println!("{:?}", pilha);
}