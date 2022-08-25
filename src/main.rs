#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell, format};
use sysinfo::{ ProcessExt, System, SystemExt};
use std::env;


struct Arguments {
	process: String,
	memory: String,
	system: String
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let mut options: Arguments = Arguments {
		process: String::from("25"),
		system: String::from("enabled"),
		memory: String::from("enabled"),
	};

	let mut	nproc = String::from("0");
	for (i,arg) in args.iter().enumerate() {
		if i > 0 {
			if *arg == String::from("-help") || *arg == String::from("help") || *arg == String::from("--help") {
				println!("jaja eu faco esse comando");
				return;
			}
			else if *arg == String::from("-s") {
				options.system = args[i].clone();
			} 
			else if *arg == String::from("-m") {
				options.memory = args[i].clone();
			}
			else if *arg == String::from("-p") {
				options.process = args[i + 1].clone();
				nproc = args[i + 1].clone();
			}
			else if *arg == nproc {}
			else {
				println!("Invalid option {}. Type 'cargo run -- -help' to see valid options.", args[i]);
				return;
			}
		}
	}

	let mut sys: System = System::new_all();
	sys.refresh_all();

	print_os_status(sys, options); 
}

fn print_os_status(sys: System, options: Arguments) {	
	// println!("------------------ disks ------------------");
	// for disk in sys.disks() {
	// 	println!("{:?}", disk);	
	// }
	// println!("------------------ network ------------------");
	// for (interface_name, data) in sys.networks() {
	// 	println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
	// }
	println!("########################################################");
	println!("#                   system information                 #");
	println!("########################################################");
	println!();
	

	
	// table.add_row(row!["total memory", sys.total_memory()]);
	// let mut table = Table::new();
	// table.add_row(row!["used memory", sys.used_memory()]);
	// table.add_row(row!["total swap", sys.total_swap()]);
	// table.add_row(row!["used swap", sys.used_swap()]);

	if options.system != "-s" {
		println!("System name:             {:?}", sys.name());
		println!("System kernel version:   {:?}", sys.kernel_version());
		println!("System OS version:       {:?}", sys.os_version());
		println!("System host name:        {:?}", sys.host_name());
		println!("N CPUs: {}", sys.cpus().len());
	}
	if options.memory != "-m" {
		println!("total memory:            {} KB", sys.total_memory());   
		println!("used memory:             {} KB", sys.used_memory());
		println!("total swap:              {} KB", sys.total_swap());
		println!("used swap:               {} KB", sys.used_swap());
	}
	println!();
	process_table(sys, options.process);
}

fn process_table(sys: System, nproc: String ) {
	let mut table = Table::new();
	table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
	table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

	table.set_titles(Row::new(vec![
		Cell::new("  PID  ").style_spec("BcFdc"),
		Cell::new("  Process Name  ").style_spec("BcFdc"),
		Cell::new("  CPU  ").style_spec("BcFdc"),
		Cell::new("  Disk  ").style_spec("BcFdc"),
		Cell::new("  VMemory  ").style_spec("BcFdc"),
		Cell::new("  Run Time  ").style_spec("BcFdc")
		// Cell::new("  Command  ").style_spec("BcFdc")
	]));
	
	let mut i = 0;
	for (pid, process) in sys.processes() {
		let proc = nproc.parse::<i32>().unwrap();

		i += 1;
		table.add_row(row![pid, process.name(), process.cpu_usage(), process.disk_usage().total_read_bytes, process.virtual_memory(), format!("{}m", process.run_time()/60)]);
		if i > proc { break };
	}
	table.printstd();
}