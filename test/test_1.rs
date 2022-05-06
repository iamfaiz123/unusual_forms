//this is blackbox testing , so in future if we migrate our backend logic to other languge , our project will 
//still be useful
use mylib::*;
use std::net::TcpListener;

pub struct App{
    URL:String


}


pub async fn spawnApp()->App
{
    let settings = get_configuration();
    let address = format!("{}:{}", setting.host, setting.port);
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to get port");
    let port = listener.local_addr().unwrap().port();
    let URL = format!("http://127.0.0.1:{}", port);
    let mut client_options = ClientOptions::parse("mongodb+srv://faizal:123@cluster0.sh3np.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").await.expect("fail to connect to the mongodb database server");
    client_options.app_name = Some(app_setting.dataBaseSetting.name);
    let client = Client::with_options(client_options).expect(" ");
    let server = run(listener,client);
    let _ = tokio::spawn(server);
    App{URL}
    
}

#[tokio::test]
fn check_routes_apis()
{
  let App = spawnApp();


}