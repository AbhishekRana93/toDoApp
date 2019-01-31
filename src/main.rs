mod prod_cons_mutex;
mod reader_writer;

fn main() {
    reader_writer::start();
    prod_cons_mutex::start();
}
