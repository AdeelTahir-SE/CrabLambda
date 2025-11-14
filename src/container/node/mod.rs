pub mod deployer;
pub mod wrapper;

use deployer::initialize_node_container;
use wrapper::node_code_wrapper;

pub fn deploy_node_container(language:&str,function:&str,dependencies:Vec<&str>,port:&str)->Result<()>{
    let wrapped_function=node_code_wrapper(function,port);
    initialize_node_container(dependencies, port, &wrapped_function)?;
    Ok(())
}