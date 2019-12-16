use crate::command::{handle_command, Command};
use brinkstore::ctx::BrinkStoreContext;
use brinkstore::store::block::BrinkBlock;
use brinkstore::store::index::search::{BrinkIndexSearch, BrinkIndexSearchKey};
use brinkstore::store::index::{BrinkIndex, BrinkIndexStore};
use brinkstore::store::loader::BrinkStoreLoader;
use brinkstore::store::BrinkStore;
use clap::{App, Arg, SubCommand};
use std::collections::HashMap;
use std::error::Error;

extern crate brinkstore;
extern crate clap;
extern crate serde_json;
extern crate tokio;

pub mod command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = App::new("brinkdb-cli")
        .version("1.0")
        .author("Leon <ljph@outlook.com>")
        .about("Command line utility to access a local or remote brink database")
        .arg(
            Arg::with_name("store")
                .help("which store should brinkdb-cli access?")
                .short("s")
                .long("store")
                .index(1)
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("server-addr")
                .help("brinkdb server connection details")
                .short("a")
                .long("server-addr")
                .takes_value(true),
        )
        .subcommand(
            App::new("index")
                .about("Index util")
                .subcommand(
                    App::new("get")
                        .about("Gets information for a certain index")
                        .arg(Arg::with_name("key").help("the key to get").index(1)),
                )
                .subcommand(
                    App::new("set")
                        .about("Sets an index")
                        .arg(
                            Arg::with_name("key")
                                .help("the key to get")
                                .index(1)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("selector")
                                .help("the JSON selector to map index values")
                                .index(2)
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            App::new("get").about("Gets a value by key").arg(
                Arg::with_name("key")
                    .help("the key to get")
                    .index(1)
                    .required(true),
            ),
        )
        .subcommand(
            App::new("del").about("Delete a value by key").arg(
                Arg::with_name("key")
                    .help("the key to delete")
                    .index(1)
                    .required(true),
            ),
        )
        .subcommand(
            App::new("set").about("Sets a value by key").args(&[
                Arg::with_name("key")
                    .help("the key to set")
                    .takes_value(true)
                    .required(true),
                Arg::with_name("value")
                    .help("the value to set")
                    .takes_value(true)
                    .required(true)
                    .multiple(true),
            ]),
        )
        .subcommand(App::new("metadata").about("Gets metadata for chosen store"))
        .get_matches();

    //    println!("{:?}", &args);
    let store_name: String = args.value_of("store").unwrap().into();
    let (subcommand, subcmd_args) = args.subcommand();
    let args = subcmd_args.unwrap();
    let command = match subcommand {
        "set" => {
            let values: Vec<String> = args
                .values_of("value")
                .unwrap()
                .map(|s| s.to_string())
                .collect();

            Command::Set {
                key: args.value_of("key").unwrap().into(),
                value: values.join(" "),
            }
        }
        "get" => Command::Get(args.value_of("key").unwrap().into()),
        "del" => Command::Delete(args.value_of("key").unwrap().into()),
        "index" => match args.subcommand() {
            ("get", args) => Command::IndexGet(
                args.unwrap()
                    .value_of("key")
                    .map_or(None, |s| Some(s.to_string())),
            ),
            _ => Command::Unknown,
        },
        "metadata" => Command::Metadata,
        _ => Command::Unknown,
    };

    let mut ctx = BrinkStoreContext::new();
    let mut block = BrinkBlock::new(1).await?;

    let mut indexes = BrinkIndexStore::new();
    indexes.add(BrinkIndex {
        key: "name".into(),
        json_selector: "$.name".into(),
    });

    indexes.add(BrinkIndex {
        key: "email".into(),
        json_selector: "$.email".into(),
    });

    indexes.add(BrinkIndex {
        key: "country".into(),
        json_selector: "$.country".into(),
    });

    ctx.add_store(match BrinkStoreLoader::read(store_name.clone()).await {
        Result::Ok(store) => store,
        Result::Err(_) => BrinkStore {
            name: store_name.clone(),
            keys: HashMap::new(),
            indexes,
        },
    });

    ctx.add_block(block);
    ctx.set_default_block(1);

    handle_command(store_name.clone(), command, &mut ctx).await?;

    let store = ctx.get_store(&store_name).unwrap();
    BrinkIndexSearch::new(vec![
        //        BrinkIndexSearchKey::new("name".into(), "Leon".into()),
        BrinkIndexSearchKey::new("country".into(), "UK".into()),
    ])
    .search(&store.indexes);

    BrinkStoreLoader::write(store).await?;

    Ok(())
}
