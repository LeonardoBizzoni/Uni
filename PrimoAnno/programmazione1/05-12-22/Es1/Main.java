import java.util.Arrays;
import java.util.Scanner;

/*
 * Mon Dec  5 18:15:41 2022
 */
public class Main {
	public static void main(String[] args) {
		Scanner input = new Scanner(System.in);
		int lastGame = 0;
		Game[] games = new Game[5];

		while (true) {
			int rand = Metodi.estraiNumeroCasuale();
			int guess, res, guess_counter = 0;
			int min_of_greater = -1, max_of_lesser = -1;

			System.out.println("Da indovinare: " + rand);
			if (lastGame == 5) {
				games = Arrays.copyOfRange(games, 1, 5);
				games = Arrays.copyOf(games, 5);
				lastGame--;
			} 
			games[lastGame] = new Game(rand);

			while (guess_counter < 10) {
				System.out.println("[TRY " + (guess_counter+1) + "]Inserisci un numero: ");
				guess = input.nextInt();

				if (guess > 1 && guess < 100) {
					guess_counter++;
					res = Metodi.confrontaNumeri(guess, rand);

					if (res == 0) {
						System.out.println("Hai indovinato");
						games[lastGame].setDid_guess(true);
						break;
					} else if (res == -1) {
						System.out.println("piu' piccolo di quello da indovinare");

						if (max_of_lesser == -1) {
							max_of_lesser = guess;
						} else if (guess < max_of_lesser) {
							System.out.println("gia' ti ho detto che era piu' grande di " + max_of_lesser);
						} else {
							max_of_lesser = guess;
						}
					} else {
						System.out.println("piu' grande del numero da indovinare");

						if (min_of_greater == -1) {
							min_of_greater = guess;
						} else if (guess > min_of_greater) {
							System.out.println("gia' ti ho detto che era piu' piccolo di " + min_of_greater);
						} else {
							min_of_greater = guess;
						}
					}
				}
			}

			games[lastGame].setN_of_guesses(guess_counter);

			for (Game game : games) {
				if(game != null)
					System.out.println("\t\t"+game.getSecret_num());
			}			
			lastGame++;

			System.out.println("Continua?");
			String continua = input.next();
			if(continua.contains("no")){
				break;
			}
		}

		input.close();
	}
}

class Metodi {
	public static int estraiNumeroCasuale() {
		return (int) (Math.random() * 100);
	}

	public static int confrontaNumeri(int numeroInserito, int numeroDaIndovinare) {
		if (numeroInserito == numeroDaIndovinare) {
			return 0;
		} else if (numeroInserito < numeroDaIndovinare) {
			return -1;
		} else {
			return 1;
		}
	}
}

class Game {
	private int secret_num;
	private boolean did_guess;
	private int n_of_guesses;

	public Game(int secret_num) {
		this.secret_num = secret_num;
		this.did_guess = false;
	}

	public int getSecret_num() {
		return secret_num;
	}
	public void setSecret_num(int secret_num) {
		this.secret_num = secret_num;
	}

	public boolean isDid_guess() {
		return did_guess;
	}
	public void setDid_guess(boolean did_guess) {
		this.did_guess = did_guess;
	}

	public int getN_of_guesses() {
		return n_of_guesses;
	}
	public void setN_of_guesses(int n_of_guesses) {
		this.n_of_guesses = n_of_guesses;
	}
}
