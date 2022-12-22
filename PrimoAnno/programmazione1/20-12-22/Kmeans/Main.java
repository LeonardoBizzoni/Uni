import java.util.Scanner;

/*
 * Tue Dec 20 17:47:13 2022
 */
public class Main {
	private static final int K = 3;
	private static final double ALPHA = 0.1;
	private static final int ITER = 1000;

	public static void main(String[] args) {
		Scanner input = new Scanner(System.in);
		int numero_iterazioni = 0;

		int m, n;

		System.out.print("Inserisci m,n: ");
		m = input.nextInt();
		n = input.nextInt();

		double[][] dati = new double[m][n];
		int[] cluster = new int[m];
		double[][] centri = new double[K][n];
		double obittivo;

		inizializzaDati(dati);

		System.out.println("---Dati---");
		for (int i=0; i<dati.length; i++) {
			for (int j=0; j<dati[i].length; j++) {
				System.out.print(dati[i][j] + " ");
			}
			System.out.println();
		}

		inizializzaCluster(centri, dati);
		System.out.println("---Centri---");
		for (int i=0; i<centri.length; i++) {
			for (int j=0; j<centri[i].length; j++) {
				System.out.print(centri[i][j] + " ");
			}
			System.out.println();
		}

		do {
			calcolaCluster(cluster, dati);
			// aggiornaCentri();
			// calcolaObbiettivo();
		} while((++numero_iterazioni < ITER));

		input.close();
	}

	private static void inizializzaDati(double[][] dati) {
		for (int i = 0; i < dati.length; i++) {
			for (int j = 0; j < dati[i].length; j++) {
				dati[i][j] = Math.random();
			}
		}
	}

	private static void inizializzaCluster(double[][] centri, double[][] dati) {
		int[] used = new int[K];
		for (int i = 0; i < used.length; i++) {
			used[i] = -1;
		}

		int new_centro = (int) Math.floor(Math.random() * dati.length);

		for (int i = 0; i < K; i++) {
			for (int j=0; j<used.length; j++) {
				if (used[j] == new_centro) {
					new_centro = (int) Math.floor(Math.random() * dati.length);
					j = 0;
				}
			}
			used[i] = new_centro;

			centri[i] = dati[new_centro];
		}
	}

	private static void calcolaCluster(int[] cluster, double[][] dati) {
		for (var oggetto : dati) {
			
		}
	}
}
