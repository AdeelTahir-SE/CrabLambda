use crate::db::file_db::update_file_functions::update_table_function_port_no;
use crate::utils::operating_system::is_port_available;
use crate::container::node::deployer;
pub fn initialize_container_deploayment(language:&str,function:&str,dependencies:Vec<&str>){
    let mut port=3000;
    loop{
        if is_port_available(port.to_string().as_str()){
            break;
        }
        else{
            port =port+1;
        }
    }


}