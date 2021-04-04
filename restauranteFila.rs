use std::io;
use std::str::FromStr;
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

fn inicio(fila1: &mut Fila::<i32>, fila2: &mut Fila::<i32>, fila3: &mut Fila::<i32>){
  let mut contador: i32 = 0;
  loop{
    let mut inputLine: String = String::new();
    println!("1 - Inserção de cliente na fila de pedidos");
    println!("2 - Remoção de cliente da fila de pedidos");
    println!("3 - Remoção de cliente da fila de pagamentos");
    println!("4 - Remoção de cliente da fila de encomendas");
    println!("5 - Vizualizar filas");
    println!("6 - Sair");
    io::stdin().read_line(&mut inputLine).unwrap();

    let number: i32 = i32::from_str(inputLine.trim()).unwrap_or(0);

    match number {
      1 => {
        contador += 1;
        fila1.enqueue(contador);
        println!("Cliente {} entrou na fila de pedidos.", contador);
      },
      2 => {
        if !fila1.is_empty(){
          let codigo: i32 = *fila1.peek().unwrap_or(&0);
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
          let codigo: i32 = *fila2.peek().unwrap_or(&0);
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
          let codigo: i32 = *fila3.peek().unwrap_or(&0);
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
      6 => break,
      _ => println!("Não temos ações para esse digito")
    }
  }
}

fn main(){
  let mut fila1: Fila<i32> = Fila::<i32>::new();
  let mut fila2: Fila<i32> = Fila::<i32>::new();
  let mut fila3: Fila<i32> = Fila::<i32>::new();
  inicio(&mut fila1, &mut fila2, &mut fila3);
}