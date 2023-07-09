use clap::{Subcommand, ValueEnum};

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ProjectsTypes {
	Folder,
	Project,
	SubProject
}

#[derive(Debug, Subcommand)]
pub enum Commands {
	#[command(arg_required_else_help = true)]
	Open { 
        project: String,
	},
	#[command(arg_required_else_help = true)]
	New {
		project: String	
	},
	#[command(arg_required_else_help = true)]
	Dir {
		project: String
	},

	List {
		#[arg(short, long)]
		flat : bool,
		#[arg(short , long)]
		r#type : Option<ProjectsTypes>,
		#[arg(short, long)]
		max_depth : Option<i32>
	},
	Run {
		#[arg(short, long)]
		project : Option<String>
	}
}