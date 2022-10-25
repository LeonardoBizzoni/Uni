import java.util.Scanner;

/*
 * Tue Oct 18 15:09:58 2022
 */
public class cartaOro1 {
	public static void main(String[] args) {
		Scanner input = new Scanner(System.in);
		long n, an = 2;

		do {
			System.out.print("Inserisci un numero naturale: ");
			n = input.nextLong();
		} while (n < 0);

		for (int i = 0; i < n; i++) {
			an = 3 * an - 2;
		}

		System.out.println(an);
		input.close();
	}
}
