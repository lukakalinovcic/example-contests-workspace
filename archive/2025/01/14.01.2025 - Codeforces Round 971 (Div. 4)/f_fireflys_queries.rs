//{"name":"F. Firefly's Queries","group":"Codeforces - Codeforces Round 971 (Div. 4)","url":"https://codeforces.com/contest/2009/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 3\n1 2 3\n1 9\n3 5\n8 8\n5 5\n4 8 3 2 4\n1 14\n3 7\n7 10\n2 11\n1 25\n1 1\n6\n1 1\n5 7\n3 1 6 10 4\n3 21\n6 17\n2 2\n1 5\n1 14\n9 15\n12 13\n5 3\n4 9 10 10 1\n20 25\n3 11\n20 22\n","output":"18\n8\n1\n55\n20\n13\n41\n105\n6\n96\n62\n1\n24\n71\n31\n14\n44\n65\n15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FFireflysQueries"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let q = input.read();
    let a = input.read_int_vec(n);
    let mut psum = vec![0; 2 * n + 1];
    for i in 0..n {
        psum[i + 1] = psum[i] + a[i] as i64;
    }
    for i in 0..n {
        psum[n + i + 1] = psum[n + i] + a[i] as i64;
    }
    let row_sum = psum[n];
    for (lo, hi) in input.read_long_pair_vec(q) {
        let lo = lo as usize;
        let hi = hi as usize;
        let (r1, c1) = ((lo - 1) / n, (lo - 1) % n);
        let (r2, c2) = ((hi - 1) / n, (hi - 1) % n);
        let mut result = 0;
        if r1 == r2 {
            result += psum[r1 + c2 + 1] - psum[r1 + c1];
        } else {
            let k = (r2 - r1 - 1) as i64;
            result += k * row_sum;
            result += psum[r1 + n] - psum[r1 + c1];
            result += psum[r2 + c2 + 1] - psum[r2];
        }
        out.print_line(result);
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
