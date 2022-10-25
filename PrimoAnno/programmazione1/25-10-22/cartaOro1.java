import java.util.Scanner;
import java.util.Stack;

/*
 * Tue Oct 25 08:13:56 2022
 */
public class cartaOro1 {
	public static void main(String[] args) {
		Scanner in = new Scanner(System.in);
		System.out.println("Inserisci un'espressione:");
		String eq = in.next();

		try {
			if (checkSyntax(new String(eq))) {
				eval(eq);
			}
		} catch (Exception error) {
			System.out.println(error);
		}

		in.close();
	}

	private static void eval(String eq) {
		Stack<Integer> nums = new Stack<>();
		Stack<Character> ops = new Stack<>();
		char[] token = eq.toCharArray();

		for (int i = 0; i < token.length; i++) {
			if (token[i] >= '0' && token[i] <= '9') {
				StringBuffer n = new StringBuffer();

				while (i < token.length && (token[i] >= '0' && token[i] <= '9')) {
					n.append(token[i++]);
				}

				nums.add(Integer.parseInt(n.toString()));
				i--;
			} else if (token[i] == '+' || token[i] == '-') {
				while (!ops.empty()) {
					runOp(ops, nums);
				}
				ops.push(token[i]);
			}
		}

		while (!ops.empty()) {
			runOp(ops, nums);
		}

		System.out.println("res: \t" + nums.pop());
	}

	private static void runOp(Stack<Character> ops, Stack<Integer> nums) {
		char op = ops.pop();
		int n1 = nums.pop();
		int n2 = nums.pop();

		if (op == '+') {
			nums.add(n1 + n2);
		} else if (op == '-') {
			nums.add(n2 - n1);
		}
	}

	private static boolean checkSyntax(String eq) throws Exception {
		if (eq.replaceAll("\\d+(-|\\+|$)", "").equals("")) {
			return true;
		} else {
			throw new Error("Invalid syntax.");
		}
	}
}
