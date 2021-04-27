use std::io;
use std::str::FromStr;
use std::process;

#[derive(Debug)]
struct Fila<T> {
  fila: Vec<T>
}

impl<T> Fila<T> {
  fn new() -> Self {
    Fila { fila: Vec::new() }
  }

  fn length(&self) -> usize {
    self.fila.len()
  }

  fn enqueue(&mut self, item: T) {
    self.fila.push(item)
  }

  fn dequeue(&mut self) -> T {
    self.fila.remove(0)
  }
  fn is_empty(&self) -> bool {
    self.fila.is_empty()
  }

  fn peek(&self) -> Option<&T> {
    self.fila.first()
  }
}

fn inicio(fila1: &mut Fila::<u32>, fila2: &mut Fila::<u32>, fila3: &mut Fila::<u32>){
  let mut contador: u32 = 0;
  let mut inputLine: String = String::new();
    
  loop{
    println!("1 - Inserção de cliente na fila de pedidos");
    println!("2 - Remoção de cliente da fila de pedidos");
    println!("3 - Remoção de cliente da fila de pagamentos");
    println!("4 - Remoção de cliente da fila de encomendas");
    println!("5 - Vizualizar filas");
    println!("6 - Sair");

    inputLine.clear();
    io::stdin().read_line(&mut inputLine).unwrap();

    let number: u32 = u32::from_str(inputLine.trim()).unwrap_or(0);

    match number {
      1 => {
        contador += 1;
        fila1.enqueue(contador);
        println!("Cliente {} entrou na fila de pedidos.", contador);
      },
      2 => {
        if !fila1.is_empty(){
          let codigo: u32 = *fila1.peek().unwrap_or(&0);
          fila1.dequeue();
          println!("Cliente {} foi removido da fila de pedidos", codigo);
          fila2.enqueue(codigo);
          println!("Cliente {} entrou na fila de pagamentos.", codigo);
        }else{
          println!("Não pode inserir na fila 2 se não tem codigo na fila 1");
        }
      },
      3 => {
        if !fila2.is_empty(){
          let codigo: u32 = *fila2.peek().unwrap_or(&0);
          fila2.dequeue();
          println!("Cliente {} foi removido da fila de pagamentos", codigo);
          fila3.enqueue(codigo);
          println!("Cliente {} entrou na fila de encomenda.", codigo);
        }else{
          println!("Não pode inserir na fila 3 se não tem codigo na fila 2");
        }
      },
      4 => {
        if !fila3.is_empty(){
          let codigo: u32 = *fila3.peek().unwrap_or(&0);
          fila3.dequeue();
          println!("Cliente {} foi removido da fila de encomendas.", codigo);
        }else{
          println!("Não pode retirar da fila 3 se ela está vazia");
        }
      },
      5 => {
        println!("Fila pedidos: {:?}", fila1);
        println!("Fila pagamentos: {:?}", fila2);
        println!("Fila encomendas: {:?}", fila3);
      }
      6 => {
        println!("Saindo!");
        process::exit(0);
        break;
      },
      _ => println!("Não temos ações para esse digito")
    }
  }
}

fn main(){
  let mut fila1: Fila<u32> = Fila::<u32>::new();
  let mut fila2: Fila<u32> = Fila::<u32>::new();
  let mut fila3: Fila<u32> = Fila::<u32>::new();
  inicio(&mut fila1, &mut fila2, &mut fila3);
}