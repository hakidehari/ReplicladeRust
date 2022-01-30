// module which contains all prompts executed within Repliclade
pub mod prompts {

    use std::io;

    // prompts for a fasta filename
    pub fn prompt_filename() -> String {
        println!("Please enter the name of the Fasta file to process: ");
        
        let mut filename = String::new();
        
        io::stdin()
            .read_line(&mut filename)
            .expect("Failed to read line.  Please enter the name of the Fasta file to process");

        while filename.trim().is_empty() {
            io::stdin()
                .read_line(&mut filename)
                .expect("Failed to read line. Please enter the name of the Fasta file to process");
        }

        return filename;
    }

}