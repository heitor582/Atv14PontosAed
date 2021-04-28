use std::io;
use std::process;
use std::str::FromStr;

#[derive(Clone)]
#[derive(Debug)]
struct Node {
    chave: u32,
    nome: String,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, chave: u32, nome: String) {
        self.head = Some(Box::new(Node {
            chave: chave,
            nome: nome,
            next: self.head.take(),
        }));

        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<u32> {
        self.head.take().map(|h| {
            self.head = h.next;
            self.size -= 1;
            h.chave
        })
    }

    pub fn findByKey(&self, chave: u32) -> Option<Box<Node>>{
        let mut aux = self.head.clone();
        loop{
            match aux{
                Some(ref h) => {
                    if h.chave == chave{
                        break;
                    }
                    aux = h.next.clone()
                },
                None => break,
            };
        }        
        
        aux
    }

    pub fn findByName(&self, nome: String) -> Option<Box<Node>>{
        let mut aux = self.head.clone();
        loop{
            match aux{
                Some(ref h) => {
                    if h.nome == nome{
                        break;
                    }
                    aux = h.next.clone()
                },
                None => break,
            };
        }        
        
        aux
    }
}

fn iniciar(linked_list: &mut LinkedList){
    let mut inputLine: String = String::new();
    let mut contador: u32 = 0;
    
    loop{
        println!("1 Pesquisar por chave");
        println!("2 Pesquisar por nome");
        println!("3 Inserir");
        println!("4 Imprimir Lista");
        println!("5 Tamanho da lista");
        println!("6 Sair");

        inputLine.clear();
        io::stdin().read_line(&mut inputLine).unwrap();
        let number: u32 = u32::from_str(inputLine.trim()).unwrap_or(0);

        match number{
            1 => {
                inputLine.clear();
                println!("Digite a chave: ");
                io::stdin().read_line(&mut inputLine).unwrap();
                let number: u32 = u32::from_str(inputLine.trim()).unwrap_or(0);
                let node = linked_list.findByKey(number);
                match node {
                    Some(node) => println!("Chave: {:?}, Node: {:#?}", number, node),
                    None => println!("Não existe"),
                }
            },
            2 => {
                inputLine.clear();
                println!("Digite o nome: ");
                io::stdin().read_line(&mut inputLine).unwrap();
                let node = linked_list.findByName((*inputLine.trim()).to_string());
                match node {
                    Some(node) => println!("Nome: {:?}, Node: {:#?}", inputLine, node),
                    None => println!("Não existe"),
                }
            },
            3 => {
                contador += 1;
                inputLine.clear();
                println!("Digite o nome: ");
                io::stdin().read_line(&mut inputLine).unwrap();
                linked_list.push(contador, (*inputLine.trim()).to_string());
            },
            4 => println!("{:#?}", linked_list),
            5 => println!("Tamanho da lista: {:?}", linked_list.size),
            6 => {                
                println!("Saindo!");
                process::exit(0);
            },
            _ => println!("Não temos ações para esse digito")
        }
    }
}

fn main() {
    let mut linked_list: LinkedList = LinkedList::new();
    iniciar(&mut linked_list)
}