use std::cmp::Ordering;
use std::fmt;
use std::io;
use std::io::Stdin;
use std::process::exit;

type GameNum = f32;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Op {
    pub fn presidense(self)->u8 {
        match self {
            Op::Plus=>0,
            Op::Minus=>0,
            Op::Multiply=>1,
            Op::Divide=>1,
        }
    }
    pub fn calc(self, f: GameNum, s: GameNum)->Option<GameNum> {
        match self {
            Op::Plus=>Some(f+s),
            Op::Minus=>Some(f-s),
            Op::Multiply=>Some(f*s),
            Op::Divide=>if s == 0.0 /*|| f % s != 0*/ { None } else { Some(f/s) },
        }
    }
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Plus=>write!(f, "+"),
            Op::Minus=>write!(f, "-"),
            Op::Multiply=>write!(f, "*"),
            Op::Divide=>write!(f, "/"),
        }
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
struct Solution(pub Vec<GameNum>, pub Vec<Op>, pub usize, pub usize);

impl fmt::Debug for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.1.len() {
            if i == self.2 { write!(f, "(")?; }
            write!(f, "{}", self.0[i])?; //num
            if i == self.3 { write!(f, ")")?; }
            write!(f, "{:?}", self.1[i])?; //op
        }
        write!(f, "{}{}", *self.0.last().unwrap(), if self.3 == self.0.len()-1 { ")" } else { "" })
    }
}

impl Solution {
    fn before(&self, other: &Solution)->Ordering {
        // i don't really care which one is less or greater, i just need the equal ones to be equal (and i guess be consistent with general order)
        if self.2 < other.2 { return Ordering::Greater; }
        if self.2 > other.2 { return Ordering::Less; }
        //lpar are equal
        if self.3 < other.3 { return Ordering::Greater; }
        if self.3 > other.3 { return Ordering::Less; }
        //rpar are equal
        if self.1 < other.1 { return Ordering::Greater; }
        if self.1 > other.1 { return Ordering::Less; }
        //ops are same
        if self.0 < other.0 { return Ordering::Greater; }
        if self.0 > other.0 { return Ordering::Less; }
        return Ordering::Equal;
    }
}

fn main() {
    let mut number_count = 4; //standard is 4
    let mut number_to_find = 10.0; //standard is 10
    let mut allowed_operators = vec![Op::Plus, Op::Minus, Op::Multiply, Op::Divide];
    let mut all_solutions = false;

    let args: Vec<String> = std::env::args().collect();
    
    //check for options -h/-help/help/h/--help/--h
    if args.contains(&"h".to_owned()) || args.contains(&"help".to_owned()) || args.contains(&"-h".to_owned()) || args.contains(&"-help".to_owned()) || args.contains(&"--h".to_owned()) || args.contains(&"--help".to_owned()) {
        println!(
"solver: I am small command line tool for solving (a slightly generalized version of) game '4=10' by Sveinn Steinarsson. Args:

-n <int>   - specify Number of numbers to read and use; default = 4
-f <float> - what number to Find; default = 10.0
-o <str>   - what Operators are allowed; default = +-*/
             use any string of +-*/, representing corresponding allowed operators (e.g. '+', '+*', '+-/')
-a         - give All solutions; by default gives a single solution
-h - show this Helpful usage guide =)

Example usage: ./solver <enter> 1 <enter> 2 <enter> 3 <enter> 5 <enter>
Example usage: ./solver -a -n 3 -f 5 -o +-* <enter> 1 <enter> 2 <enter> 3 <enter>");
        exit(0);
    }
    //check for option -n - specify amount of numbers to be read; default is 4
    if let Some(num_arg) = args.iter().position(|r| r == "-n") {
        if num_arg+1 == args.len() {
            println!("solver: There is no number given to option '-n'.");
            exit(1);
        } else {
            let num = args[num_arg+1].parse();// GameNum::from_str_radix(&input[..len-1], 10);
            if let Ok(num) = num {
                number_count = num;
            } else {
                println!("solver: Argument after option '-n' is not a number: {}", args[num_arg+1]);
                exit(1);
            }
        }
    }
    //check for option -f - specify what number to find; default is 10.0
    if let Some(num_arg) = args.iter().position(|r| r == "-f") {
        if num_arg+1 == args.len() {
            println!("solver: There is no number given to option '-f'.");
            exit(1);
        } else {
            let num = args[num_arg+1].parse();// GameNum::from_str_radix(&input[..len-1], 10);
            if let Ok(num) = num {
                number_to_find = num;
            } else {
                println!("solver: Argument after option '-f' is not a number: {}", args[num_arg+1]);
                exit(1);
            }
        }
    }
    //check for option -o - specify what operators are allowed; default is +-*/
    if let Some(num_arg) = args.iter().position(|r| r == "-o") {
        if num_arg+1 == args.len() {
            println!("solver: There is no number given to option '-o'.");
            exit(1);
        } else {
            allowed_operators = vec![];
            for op in args[num_arg+1].chars() {
                match op {
                    '+' => allowed_operators.push(Op::Plus),
                    '-' => allowed_operators.push(Op::Minus),
                    '*' => allowed_operators.push(Op::Multiply),
                    '/' => allowed_operators.push(Op::Divide),
                    _ => {
                        println!("solver: Argument after option '-o' is not a valid operator list: {}", args[num_arg+1]);
                        exit(1);
                    }
                }
            }
            //make sure there are no duplicate signs
            allowed_operators.sort();
            allowed_operators.dedup_by(|a, b| a == b);
        }
    }
    //check for option -a - give all possible solutions
    if args.contains(&"-a".to_owned()) {
        all_solutions = true;
    }

    //read input
    let stdin = io::stdin();
    let mut nums = vec![];
    for _ in 0..number_count {
        if let Some(num) = read_num(&stdin) {
            nums.push(num);
        } else {
            println!("solver: Not a valid number.");
            exit(1);
        }
    }

    //solve
    println!("solver: Starting bruteforce solution...");
    let mut results = brute_force_find(nums, allowed_operators, number_to_find, all_solutions);
    
    results.sort_by(|a, b| a.before(b));
    results.dedup_by(|a, b| a == b);

    println!("solver: Solutions:");
    println!("{:?}", results);
}

fn read_num(stdin: &Stdin)->Option<GameNum> {
    let mut input = String::new();
    let res = stdin.read_line(&mut input);
    match res {
        Ok(_) => {
            let num = input.trim().parse();// GameNum::from_str_radix(&input[..len-1], 10);
            if let Ok(num) = num { return Some(num); } else { return None; };
        }
        Err(_) => return None,
    }
}

//rules: take numbers
//- can move them around
//- can use as many ops as you want
//- use 1 set of parantheses anywhere
fn brute_force_find(nums: Vec<GameNum>, ops: Vec<Op>, target: GameNum, all_solutions: bool)->Vec<Solution> {
    let mut v = vec![];
    return permute_numbers(&mut v, &nums, &ops, target, all_solutions);
}

fn permute_numbers(mut built: &mut Vec<GameNum>, remaining: &[GameNum], ops: &[Op], target: GameNum, all_solutions: bool)->Vec<Solution> {
    //base case - no numbers remaining
    if remaining.len() == 0 {
        let mut used_ops = vec![];
        let answers = permute_ops(built, &mut used_ops, ops, target, all_solutions);
        return answers;
    }
    //numbers remain
    let mut answers = vec![];
    for i in 0..remaining.len() {
        let mut new_rem = remaining.to_vec();
        built.push(new_rem.remove(i));
        let mut returned = permute_numbers(&mut built, &new_rem, ops, target, all_solutions);
        answers.append(&mut returned);
        if !all_solutions && answers.len() > 0 { break; }
        built.pop();
    }
    return answers;
}

fn permute_ops(nums: &[GameNum], mut ops: &mut Vec<Op>, allowed_ops: &[Op], target: GameNum, all_solutions: bool)->Vec<Solution> {
    //base case - full on ops
    if ops.len() == nums.len()-1 {
        let answers = permute_parantheses(nums, &ops, target, all_solutions);
        return answers;
    }
    //add more operators
    let mut answers = vec![];
    for op in allowed_ops {
        ops.push(*op);
        let mut returned = permute_ops(nums, &mut ops, allowed_ops, target, all_solutions);
        answers.append(&mut returned);
        if !all_solutions && answers.len() > 0 { break; }
        ops.pop();
    }
    return answers;
}

fn permute_parantheses(nums: &[GameNum], ops: &[Op], target: GameNum, all_solutions: bool)->Vec<Solution> {
    let mut answers = vec![];
    for i in 0..(nums.len()-1) {
        for j in (i+1)..nums.len() {
            let returned = eval(nums, ops, i, j);
            if let Some(returned) = returned {
                if returned == target {
                    let sol = Solution(nums.to_vec(), ops.to_vec(), i, j);
                    if all_solutions {
                        answers.push(sol);
                    } else {
                        //give just the first solution
                        return vec![sol];
                    }
                }
            }
        }
    }
    return answers;
}

//an unholy abomination; and very slow too =)
//lpal - before which number to put a left paranthese
//rpal - after which number to put a right paranthese
fn eval(nums: &[GameNum], ops: &[Op], lpar: usize, rpar: usize)->Option<GameNum> {
    let mut operands = vec![];
    let mut operators: Vec<Op> = vec![];
    //base case: parentheses around the whole expression (or empty)
    if nums.len() == 0 {
        return None;
    }
    if lpar == 0 && rpar == nums.len()-1 {
        if nums.len() == 1 {
            return Some(nums[0]);
        }
        //len > 1
        operands.push(nums[0]);
        for i in 0..ops.len() {
            //if same or smaller presidence as last op, do top calculation now, push after
            let new_op = ops[i];
            let new_num = nums[i+1];
            while !operators.is_empty() && new_op.presidense() <= operators.last().unwrap().presidense() {
                let s = operands.pop().unwrap();
                let f = operands.pop().unwrap();
                let result = operators.pop().unwrap().calc(f, s)?;
                operands.push(result);
            }
            operands.push(new_num);
            operators.push(new_op);
        }
        while !operators.is_empty() {
            let s = operands.pop().unwrap();
            let f = operands.pop().unwrap();
            let result = operators.pop().unwrap().calc(f, s)?;
            operands.push(result);
        }
        return Some(operands[0]);
    }
    //3 cases - lpar is 0; rpar is nums.len()-1; none of the above
    if lpar == 0 {
        //eval stuff inside of parentheses
        operands.push(eval(&nums[lpar..=rpar], &ops[lpar..rpar], 0, rpar-lpar)?);
        //eval right of parentheses
        for i in &nums[rpar+1..] {
            operands.push(*i);
        }
        for i in &ops[rpar..] {
            operators.push(*i);
        }
        return eval(&operands, &operators, 0, operands.len()-1);
    } else if rpar == nums.len()-1 {
        //eval left of parentheses
        for i in &nums[0..lpar] {
            operands.push(*i);
        }
        for i in &ops[0..lpar] {
            operators.push(*i);
        }
        operands.push(eval(&nums[lpar..=rpar], &ops[lpar..rpar], 0, rpar-lpar)?);
        return eval(&operands, &operators, 0, operands.len()-1);
    } else {
        //eval left of parentheses
        for i in &nums[0..lpar] {
            operands.push(*i);
        }
        for i in &ops[0..lpar] {
            operators.push(*i);
        }
        //eval stuff inside of parentheses
        operands.push(eval(&nums[lpar..=rpar], &ops[lpar..rpar], 0, rpar-lpar)?);
        //eval right of parentheses
        for i in &nums[rpar+1..] {
            operands.push(*i);
        }
        for i in &ops[rpar..] {
            operators.push(*i);
        }
        return eval(&operands, &operators, 0, operands.len()-1);
    }
}