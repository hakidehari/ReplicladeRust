pub mod prompts;

pub mod simulator {

    use crate::simulator::prompts::prompts::prompt_filename;
    use bio::io::fasta::Reader;
    
    pub fn simulate() {
        let filename = prompt_filename();
        let file_reader = Reader::from_file(&filename).unwrap();
        //let mut file_contents = BufReader::new(file_reader);

        
        for record in file_reader.records() {
            let result_data = &record.unwrap();
            println!("{:?}", result_data.seq());
        }


        

    }
}



