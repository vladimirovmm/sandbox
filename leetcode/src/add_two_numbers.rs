//! Вам даны два непустых связанных списка, представляющих два положительных целых числа.
//! Цифры в них хранятся в обратном порядке, и каждый из узлов содержит одну цифру.
//! Сложите два числа и верните сумму в виде связанного списка.
//! Вы можете предположить, что эти два числа не содержат ведущих нулей, кроме самого числа 0.

#![cfg(test)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result = ListNode::new(0);
    let mut result_cur = &mut result.next;

    let mut l1_cur = &l1;
    let mut l2_cur = &l2;
    let mut ost = 0;
    loop {
        let mut v = match (l1_cur, l2_cur) {
            (Some(a), Some(b)) => {
                l1_cur = &a.next;
                l2_cur = &b.next;
                a.val + b.val + ost
            }
            (Some(a), None) => {
                l1_cur = &a.next;
                a.val + ost
            }
            (None, Some(b)) => {
                l2_cur = &b.next;
                b.val + ost
            }
            (None, None) => {
                if ost == 0 {
                    break;
                }
                ost
            }
        };
        ost = 0;
        if v > 9 {
            ost = 1;
            v -= 10;
        }

        *result_cur = Some(Box::new(ListNode::new(v)));
        result_cur = &mut result_cur.as_mut().unwrap().next;
    }
    result.next
}

fn into_list_node(v: &[i32]) -> Option<Box<ListNode>> {
    let mut result = None;
    for value in v.iter().rev() {
        result = Some(Box::new(ListNode {
            val: *value,
            next: result,
        }));
    }
    result
}

#[test]
fn test_add_two_numbers() {
    assert_eq!(
        add_two_numbers(into_list_node(&[2, 4, 3]), into_list_node(&[5, 6, 4])),
        into_list_node(&[7, 0, 8])
    );
    assert_eq!(
        add_two_numbers(into_list_node(&[2, 4]), into_list_node(&[5])),
        into_list_node(&[7, 4])
    );

    assert_eq!(
        add_two_numbers(into_list_node(&[0]), into_list_node(&[0])),
        into_list_node(&[0])
    );

    assert_eq!(
        add_two_numbers(
            into_list_node(&[9, 9, 9, 9, 9, 9, 9]),
            into_list_node(&[9, 9, 9, 9])
        ),
        into_list_node(&[8, 9, 9, 9, 0, 0, 0, 1])
    );
}
