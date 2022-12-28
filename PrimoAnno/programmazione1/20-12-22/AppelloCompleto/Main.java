/*
 * Tue Dec 20 16:20:58 2022
 */
public class Main {
	public static void main(String[] args) {
		if (args.length != 2) {
			System.out.println("Input non valido");
			System.exit(1);
		}

		String espressioni = args[0];
		String valori = args[1];
		String daCalc;
		int[] table = { 0, 0, 0, 0, 0 };

		if (!(checkEspr(espressioni) && checkValori(valori))) {
			System.exit(1);
		}

		estraiValori(valori, table);
		daCalc = sostituisci(espressioni, table);

		System.out.println(calcola(daCalc));
	}

	private static boolean checkEspr(String espressioni) {
		int _last_value = 0;
		int _last_op = 0;

		for (int i = 0; i < espressioni.length(); i++) {
			var ch = espressioni.charAt(i);

			if ((ch >= 'A' && ch <= 'E') || (ch >= '0' && ch <= '9')) {
				_last_value++;
			} else if (ch == '+' || ch == '-') {
				_last_op++;
			} else {
				return false;
			}
		}

		while (_last_op > 0) {
			_last_op--;

			_last_value--;
			_last_value--;
		}

		if (_last_value != -1) {
			return false;
		}
		return true;
	}

	private static boolean checkValori(String valori) {
		boolean prev_was_num = valori.charAt(0) >= 'A' && valori.charAt(0) <= 'E' ? true : false;
		boolean prev_was_var = false;
		boolean prev_was_colon = false;

		for (int i = 0; i < valori.length(); i++) {
			var ch = valori.charAt(i);

			if (ch >= 'A' && ch <= 'E' && prev_was_num) {
				prev_was_num = false;
				prev_was_colon = false;
				prev_was_var = true;
			} else if (ch == ':' && prev_was_var) {
				prev_was_num = false;
				prev_was_colon = true;
				prev_was_var = false;
			} else if (ch >= '0' && ch <= '9' && prev_was_colon) {
				prev_was_num = true;
				prev_was_colon = false;
				prev_was_var = false;
			} else {
				return false;
			}
		}

		return true;
	}

	private static void estraiValori(String valori, int[] table) {
		for (int i = 0; i < valori.length(); i++) {
			var ch = valori.charAt(i);

			switch (ch) {
				case 'A':
					table[0] = valori.charAt(i += 2) - '0';
					break;
				case 'B':
					table[1] = valori.charAt(i += 2) - '0';
					break;
				case 'C':
					table[2] = valori.charAt(i += 2) - '0';
					break;
				case 'D':
					table[3] = valori.charAt(i += 2) - '0';
					break;
				case 'E':
					table[4] = valori.charAt(i += 2) - '0';
					break;
			}
		}
	}

	private static String sostituisci(String espressioni, int[] table) {
		String res = "";

		for (int i = 0; i < espressioni.length(); i++) {
			var ch = espressioni.charAt(i);

			switch (ch) {
				case 'A':
					res += table[0];
					break;
				case 'B':
					res += table[1];
					break;
				case 'C':
					res += table[2];
					break;
				case 'D':
					res += table[3];
					break;
				case 'E':
					res += table[4];
					break;
				default:
					res += ch;
			}
		}

		return res;
	}

	private static int calcola(String daCalc) {
		var nums = new int[daCalc.length()];
		var op = new char[daCalc.length()];

		int res = 0;
		int _last_num = 0;
		int _last_op = 0;

		for (int i = 0; i < daCalc.length(); i++) {
			var ch = daCalc.charAt(i);

			if (ch >= '0' && ch <= '9') {
				nums[_last_num++] = ch - '0';
			} else {
				op[_last_op++] = ch;
			}
		}

		while (_last_op > 0) {
			var n1 = nums[--_last_num];
			var n2 = nums[--_last_num];

			if (op[--_last_op] == '+') {
				nums[_last_num] = n1 + n2;
			} else {
				nums[_last_num] = n2 - n1;
			}
			_last_num++;
		}

		return nums[0];
	}
}
