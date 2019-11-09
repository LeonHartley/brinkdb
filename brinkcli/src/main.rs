use std::error::Error;
use std::collections::HashMap;
use brinkstore::store::index::{BrinkIndexStore, BrinkIndex};
use brinkstore::ctx::BrinkStoreContext;
use brinkstore::store::block::BrinkBlock;
use brinkstore::store::loader::BrinkStoreLoader;
use brinkstore::store::BrinkStore;
use brinkstore::store::index::search::{BrinkIndexSearch, BrinkIndexSearchKey};
use clap::{App, SubCommand, Arg};
use crate::command::{Command, handle_command};

extern crate tokio;
extern crate brinkstore;
extern crate clap;
extern crate serde_json;

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
        .args(&[
            Arg::with_name("server-addr")
                .help("brinkdb server connection details")
                .short("a")
                .long("server-addr")
                .takes_value(true)
        ])
        .subcommand(App::new("get")
            .about("Gets a value by key")
            .arg(
                Arg::with_name("key")
                    .help("the key to get")
                    .index(1)
                    .required(true),
            ))
        .subcommand(App::new("set")
            .about("Sets a value by key")
            .args(&[
                Arg::with_name("key")
                    .help("the key to set")
                    .takes_value(true)
                    .required(true),
                Arg::with_name("value")
                    .help("the value to set")
                    .takes_value(true)
                    .required(true)
                    .raw(true)
                    .multiple(true)
            ]))
        .subcommand(App::new("metadata")
            .about("Gets metadata for chosen store"))
        .get_matches();

//    println!("{:?}", args);
    let store_name: String = args.value_of("store").unwrap().into();
    let (subcommand, subcmd_args) = args.subcommand();
    let args = subcmd_args.unwrap();
    let command = match subcommand {
        "set" => {
            let values: Vec<String> = args.values_of("value")
                .unwrap()
                .map(|s| s.to_string())
                .collect();

            Command::Set(args.value_of("key").unwrap().into(), values.join(" "))
        }
        "get" => Command::Get(args.value_of("key").unwrap().into()),
        "metadata" => Command::Metadata,
        _ => Command::Unknown
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
        }
    });

    ctx.add_block(block);
    ctx.set_default_block(1);

    handle_command(store_name.clone(), command, &mut ctx).await?;

    BrinkStoreLoader::write(ctx.get_store(&store_name).unwrap()).await?;

    Ok(())
}
