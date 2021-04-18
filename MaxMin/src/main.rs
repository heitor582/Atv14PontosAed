extern crate rand;
use rand::Rng;

fn max_min(vet: &mut [i32; 1000], cont: &mut u32){
  let mut ma = vet[0];
  let mut me = vet[0];
  *cont = 0;

  for v in vet{
    if *v < me {
      me = *v;
      *cont += 1;      
    }
    if *v > ma{
      ma = *v;
      *cont += 1;
    }
  }
}

fn max_min2(vet: &mut [i32; 1000], cont: &mut u32){
  let mut ma = vet[0];
  let mut me = vet[0];
  *cont = 0;

  for v in vet{
    if *v < me {
      me = *v;
      *cont += 1;
    }else{
      *cont += 1;
      if *v > ma{
        ma = *v;
        *cont += 1;
      }
    }
  }
}

fn max_min3(vet: &mut [i32; 1000], cont: &mut u32){
  let mut ma;
  let mut me;
  *cont = 0;

  if vet[0] < vet[1]{
    me = vet[0];
    ma = vet[1];
    *cont += 1;
  }else{
    me = vet[1];
    ma = vet[0];
    *cont += 1;
  }

  for i in 2..vet.len()/2{
    if vet[i] < vet[i + 1]{
      *cont += 1;
      if vet[i] < me{          
        me = vet[i];
        *cont += 1;
      }
      if vet[i + 1] > ma{          
        ma = vet[i + 1];
        *cont += 1;
      }
    }else{
      *cont += 1;
      if vet[i + 1] < me{
        me = vet[i + 1];
        *cont += 1;
      }            
      if vet[i] > ma{
        ma = vet[i];
        *cont += 1;
      }            
    }
  }
}

fn gerar_vetor(vet: &mut [i32; 1000], tipo: u8){
  let mut rng = rand::thread_rng();

  for i in 0..vet.len(){
    match tipo{
      1 => vet[i] = i as i32,
      2 => vet[i] = (vet.len() - i) as i32,
      3 => vet[i] = rng.gen::<i32>(),
      _ => vet[i] = 0,
    };
  }
}

fn main(){
  let mut cont: u32 = 0;
  let mut vet: [i32; 1000] = [0; 1000];

  for v in 1..10{
    let tipo: &str = match v{
      1|4|7 => "crescente",
      2|5|8 => "decrescente",
      3|6|9 => "aleatorio",
      _ => "Error",
    };
    gerar_vetor(&mut vet, 1);
    let funcao: i32 = match v{
      1|2|3 => {
        max_min(&mut vet,&mut cont);
        1
      },
      4|5|6 => {
        max_min2(&mut vet,&mut cont);
        2
      },
      7|8|9 => {
        max_min3(&mut vet,&mut cont);
        3
      },
      _ => {
        println!("Error");
        4
      }
    };
    println!("Contagem de comparação max_min{:?} tipo {:?}: {:?}", funcao, tipo ,cont);
  }
}
