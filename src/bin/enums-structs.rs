enum Protocol {
    Uniswap,
    Pancakeswap,
    Chainlink,
    Biconomy
}

struct BlockchainProtocol {
    name: Protocol,
    priority: i32
}

fn print_struct(blockchain_protocol: BlockchainProtocol) {
    match blockchain_protocol.name {
        Protocol::Uniswap => println!("Uniswap"),
        _ => println!("Handle the generic value")
    }
}

fn main() {
    let uniswap = BlockchainProtocol{
        name: Protocol::Uniswap,
        priority: 1
    };

    print_struct(uniswap);

    let pancake_swap = BlockchainProtocol{
        name: Protocol::Pancakeswap,
        priority: 2
    };

    print_struct(pancake_swap);

    let chainlink = BlockchainProtocol{
        name: Protocol::Chainlink,
        priority: 3
    };

    print_struct(chainlink);
}