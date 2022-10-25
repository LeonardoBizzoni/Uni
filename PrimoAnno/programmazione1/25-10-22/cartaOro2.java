import java.util.Scanner;

/*
 * Tue Oct 25 09:35:38 2022
 */
public class cartaOro2 {
    public static void main(String[] args) {
		Scanner in = new Scanner(System.in);
		String s1, s2;

		System.out.print("Scrivi una frase: ");
		s1 = in.nextLine();
		checkSyntax(s1, "[a-z]");

		System.out.print("Scrivi un'altra frase: ");
		s2 = in.nextLine();
		checkSyntax(s2, "[A-Z]");

		for (char c1 : s1.toCharArray()) {
			for (char c2 : s2.toCharArray()) {
				System.out.print(c1 + "" + c2 + " ");
			}
			System.out.println();
		}

		in.close();
    }

	private static void checkSyntax(String s, String reg) {
		if (!(new String(s).replaceAll(reg, "").equals(""))) {
			throw new Error("Invalid syntax");
		}
	}
}
