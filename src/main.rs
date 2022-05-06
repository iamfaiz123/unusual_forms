use mylib::*; //all defined modules used here are imported in lib.rs
use env_logger::Env;

//this is our binary entery point





#[tokio::main]
async fn main() -> std::io::Result<()> {
    //taking enviorment variables from .yaml files
    let app_setting = get_configuration().unwrap();

     //binding our network to desired port
    let address = format!("{}:{}", app_setting.host, app_setting.port);
    let listener = TcpListener::bind(address)?;

    //defining connection to database
    let mut client_options = ClientOptions::parse("mongodb+srv://faizal:123@cluster0.sh3np.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").await.expect("fail to connect tp the server");
    client_options.app_name = Some(app_setting.dataBaseSetting.name);
    env_logger::Builder:: from_env( Env:: default().default_filter_or("info")).init();
    let client = Client::with_options(client_options).expect(" ");
    let db = client.database("unusual_froms");
    

    //external function to start server
    run(listener,db)?.await
   
}
