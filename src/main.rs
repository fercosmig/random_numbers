extern crate rand;
use rand::Rng;

// add rand = "0.3" nas dependencias do arquivo Cargo.toml

fn main()
{
    {
        // numeros inteiros entre 0 e 9
        let mut meu_vetor = vec![];
        for _i in 0..40
        {
            let valor_randomico = rand::thread_rng().gen_range(0, 10);
            meu_vetor.push(valor_randomico);
        }
        println!("{:?}", meu_vetor);
    }

    {
        // numeros decimais entre 0.0 e 9.999999999999999999999999999999999999...
        let mut meu_vetor = vec![];
        for _i in 0..5
        {
            let valor_randomico = rand::thread_rng().gen_range(0., 10.);
            meu_vetor.push(valor_randomico);
        }
        println!("{:?}", meu_vetor);
    }

    {
        // valor boleado
        let mut meu_vetor = vec![];
        for _i in 0..15
        {
            let valor_randomico = rand::thread_rng().gen_weighted_bool(2);
            // 2 = 50% de chances de ser true;
            // 4 = 25% de chances de ser true;
            // 5 = 20% de chances de ser true;
            meu_vetor.push(valor_randomico);
        }
        println!("{:?}", meu_vetor);
    }
}
