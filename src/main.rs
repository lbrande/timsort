#[macro_use]
mod kattis;

fn main() {
    let (n, rest): (usize, String);
    scanln!(2, n, rest);
    let mut a: Vec<i32> = scan_str!(rest).collect();
    timsort(&mut a, n);
    let mut result = String::new();
    for z in a {
        result.push_str(&z.to_string());
        result.push(' ');
    }
    println!("{}", result);
}

fn timsort(list: &mut [i32], n: usize) {
    let minrun = compute_minrun(n);
    let mut runs = Vec::new();
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && list[j] < list[j - 1] {
            j += 1;
        }
        if j > i + 1 {
            list[i..j].reverse();
        } else {
            while j < n && list[j] >= list[j - 1] {
                j += 1;
            }
        }
        while j < n && j - i < minrun {
            let temp = list[j];
            let mut k = j;
            while k > i && temp < list[k - 1] {
                list[k] = list[k - 1];
                k -= 1;
            }
            list[k] = temp;
            j += 1;
        }
        runs.push((i, j - i));
        loop {
            let (a, b, c) = (
                runs.len().max(3) - 3,
                runs.len().max(2) - 2,
                runs.len().max(1) - 1,
            );
            if runs.len() > 2 && runs[a].1 <= runs[b].1 + runs[c].1 {
                if runs[a].1 < runs[c].1 {
                    merge(list, runs[a], runs[b]);
                    runs[b] = (runs[a].0, runs[a].1 + runs[b].1);
                    runs.remove(a);
                } else {
                    merge(list, runs[b], runs[c]);
                    runs[c] = (runs[b].0, runs[b].1 + runs[c].1);
                    runs.remove(b);
                }
            } else if runs.len() > 1 && runs[b].1 <= runs[c].1 {
                merge(list, runs[b], runs[c]);
                runs[c] = (runs[b].0, runs[b].1 + runs[c].1);
                runs.remove(b);
            } else {
                break;
            }
        }
        i = j;
    }
    while runs.len() > 1 {
        let (b, c) = (runs.len() - 2, runs.len() - 1);
        merge(list, runs[b], runs[c]);
        runs[c] = (runs[b].0, runs[b].1 + runs[c].1);
        runs.remove(b);
    }
}

fn compute_minrun(mut n: usize) -> usize {
    let mut any_set = 0;
    while n >= 64 {
        any_set |= n & 1;
        n >>= 1;
    }
    n + any_set
}

fn merge(list: &mut [i32], a: (usize, usize), b: (usize, usize)) {
    if a.1 <= b.1 {
        let mut temp = vec![0; a.1];
        temp.copy_from_slice(&list[a.0..a.0 + a.1]);
        let (mut list_pos, mut temp_pos, mut b_pos) = (a.0, 0, b.0);
        while list_pos < b_pos {
            if temp[temp_pos] <= list[b_pos] {
                list[list_pos] = temp[temp_pos];
                temp_pos += 1;
            } else {
                list[list_pos] = list[b_pos];
                b_pos += 1;
                if b_pos == b.0 + b.1 {
                    list[list_pos + 1..b_pos].copy_from_slice(&temp[temp_pos..]);
                    break;
                }
            }
            list_pos += 1;
        }
    } else {
        let mut temp = vec![0; b.1];
        temp.copy_from_slice(&list[b.0..b.0 + b.1]);
        let (mut list_pos, mut temp_pos, mut a_pos) = (b.0 + b.1 - 1, b.1 - 1, a.0 + a.1 - 1);
        while list_pos > a_pos {
            if temp[temp_pos] >= list[a_pos] {
                list[list_pos] = temp[temp_pos];
                if temp_pos > 0 {
                    temp_pos -= 1;
                }
            } else {
                list[list_pos] = list[a_pos];
                a_pos -= 1;
                if a_pos + 1 == a.0 {
                    list[a_pos + 1..list_pos].copy_from_slice(&temp[..=temp_pos]);
                    break;
                }
            }
            list_pos -= 1;
        }
    }
}
