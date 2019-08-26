#[derive(Debug)]
struct Question {
    number1: i32,
    number2: i32,
    operation: String,
 }
 fn calc(question:Question) -> i32 {
     if question.operation == "*" {
         question.number1 * question.number2
         
         }else if question.operation == "+" {
             question.number1 + question.number2
         
             }else if question.operation == "-" {
                 question.number1 - question.number2
             }else {
                 println!("Wrong operation");
                 123
             }
         
     
 }
 fn main() {
     let assignment_question1 = Question {
         number1: 50, number2: 45, operation: "-".to_string(),
     };
     println!("Find the substraction figure of this expression {:#?}", assignment_question1);
     
     let solution1 = calc(assignment_question1);
     println!("The solution to the question is {}", solution1);
     
     let assignment_question2 = Question {
         number1: 50, number2: 45, operation: "*".to_string(),
     };
     
     
     println!("Find the multiplication figure of this expression {:#?}", assignment_question2);
     let solution2 = calc(assignment_question2);
     println!("The solution is {}", solution2);
     
     let assignment_question3 = Question {
         number1: 50, number2:45, operation: "+".to_string(),
     };
     
     
     println!("Find the summation of this question {:#?}", assignment_question3);
     let solution3 = calc(assignment_question3);
     println!("The summation is {}", solution3);
 }
