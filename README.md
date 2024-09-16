# Rgit

**Rgit** is a simplified Git implementation written in **Rust**. It replicates key features of Git, such as **creating repositories**, **adding files**, **committing changes**, and working with **branches**, **tags**, and **remotes**.

## Features

- Initialize a new Git repository or reinitialize an existing one (`init`)
- Add files to the staging area (`add`)
- Commit changes to the repository (`commit`)
- Check the status of your working directory (`status`)
- Show commit history (`log`)
- Create, list, or delete branches (`branch`)
- Checkout to different branches or commits (`checkout`)
- Create, list, or delete tags (`tag`)
- Compare changes between commits or working tree (`diff`)
- Manage repository configuration (`config`)
- Manage remote repositories (`remote`)

## Installation

1. Clone the repository:

```bash
git clone https://github.com/Kei-K23/rgit.git
```

2. Navigate to the project directory:

```bash
cd rgit
```

3. Build the project:

```bash
cargo build --release
```

4. Run the rgit binary:

```bash
./target/release/rgit
```

## Usage

You can interact with rgit through various subcommands. Here are some examples:

### Initialize a Repository

```bash
./target/release/rgit init
```

This will create an empty Git repository or reinitialize an existing one.

### Add Files

```bash
./target/release/rgit add <file>
```

Stage file contents to the index.

### Commit Changes

```bash
./target/release/rgit commit -m "Your commit message"
```

Record changes to the repository with a message.

### Check Repository Status

```bash
./target/release/rgit status
```

Display the current state of the working directory and staging area.

### View Commit Log

```bash
./target/release/rgit log
```

Show the commit history of the repository.

### Create a New Branch

```bash
./target/release/rgit branch <branch_name>
```

Create a new branch.

### Checkout a Branch or Commit

```bash
./target/release/rgit checkout <branch_name|commit_hash>
```

Switch to another branch or checkout a specific commit.

### Tag Management

#### Create a new tag:

```bash
./target/release/rgit tag <tag_name>
```

#### List all tags:

```bash
./target/release/rgit tag
```

#### Delete a tag:

```bash
./target/release/rgit tag -d <tag_name>
```

### Compare Changes

```bash
./target/release/rgit diff
```

Show changes between the working directory, staged changes, and commit history.

### Configure Repository

#### Set a configuration value:

```bash
rgit config set <key> <value>
```

#### Get a configuration value:

```bash
rgit config get <key>
```

### Manage Remotes

#### Add a new remote:

```bash
rgit remote add <name> <url>
```

#### Remove a remote:

```bash
rgit remote remove <name>
```

## License

This project is licensed under the MIT License. See the [LICENSE](/LICENSE) file for details.
