pub mod prompts;

pub mod simulator {

    use crate::simulator::prompts::prompts::prompt_filename;
    use bio::io::fasta::Reader;
    
    pub fn simulate() {
        let filename = prompt_filename();
        let mut reader = Reader::from_file(&filename);

        for result in reader { 
            println!("{}", result.read_to_string());
        }
    }
}



