import java.util.Scanner;

/*
 * Tue Oct 18 15:17:26 2022
 */
public class cartaOro2 {
	public static void main(String[] args) {
		Scanner input = new Scanner(System.in);
		int n = 0;
		double epsilon;

		do {
			System.out.print("Inserisci un valore epsilon: ");
			epsilon = input.nextDouble();
		} while (epsilon < 0);

		while (!(1.0 - epsilon < (double) n / (n + 1) && (double) n / (n + 1) <= 1)) {
			n++;
		}

		System.out.println("N: " + n + " = " + (double) n / (n + 1));

		input.close();
	}
}
