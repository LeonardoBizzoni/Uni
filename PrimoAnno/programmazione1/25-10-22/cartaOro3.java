import java.util.Scanner;

/*
 * Tue Oct 25 09:45:32 2022
 */
public class cartaOro3 {
	public static void main(String[] args) {
		Scanner in = new Scanner(System.in);
		StringBuilder res = new StringBuilder();

		System.out.print("Scrivi una frase: ");
		String s = in.next();
		checkSyntax(s, "[a-z]");

		System.out.print("Scrivi 2 numeri: ");
		int n = in.nextInt();
		int m = in.nextInt();

		if ((n >= 0 && n < s.length()) && (m >= 0 && m < s.length())) {
			for (int i = 0, k = 0; i < s.length(); i++) {
				if (k == 0 && i == m) {
					res.append(s.toUpperCase().charAt(i));
					k++;
				} else if (k > 0 && i == (m + k * n) % s.length()) {
					res.append(s.toUpperCase().charAt(i));
					k++;
				} else {
					res.append(s.charAt(i));
				}
			}

			System.out.println("Res:\t" + res);
		} else {
			in.close();
			throw new Error("Invalid numbers.");
		}

		in.close();
	}

	private static void checkSyntax(String s, String reg) {
		if (!(new String(s).replaceAll(reg, "").equals(""))) {
			throw new Error("Invalid syntax.");
		}
	}
}
