pub mod prompts;

pub mod simulator {

    use crate::simulator::prompts::prompts::prompt_filename;
    use bio::io::fasta::Reader;
    use std::path::MAIN_SEPARATOR;
    
    pub fn simulate() {
        let filename = prompt_filename();
        let path_to_fasta = format!("..{}fastas{}{}", &filename);
        
        let reader_object = Reader::from_file("..{}fastas{}{}", &filename);
    }
}



