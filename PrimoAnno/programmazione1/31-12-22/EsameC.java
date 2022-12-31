import java.util.Scanner;

class EsameC {
  public static void main(String[] args) {
    Scanner input = new Scanner(System.in);
    int n, a, b, counter = 0;

    System.out.println("Inserisci n:");
    do {
      n = input.nextInt();
    } while (n<=0);

    System.out.println("Inserisci valori di interi:");
    int[] interi = new int[n];
    for (int i=0; i<interi.length; i++) {
      interi[i] = input.nextInt();
    }

    System.out.println("Inserisci a<b:");
    do {
      a = input.nextInt();
      b = input.nextInt();
    } while (a>=b);

    int[] copia_interi = new int[n];
    for (int i=0; i<copia_interi.length; i++) {
      copia_interi[i] = interi[i];
    }

    Metodi.occorrenzaElementi(copia_interi, a, b);

    System.out.print("Interi: ");
    for (int i=0; i<interi.length; i++) {
      System.out.print(interi[i]+ " ");
    }
    System.out.print("\nCopia: ");

    for (int i=0; i<copia_interi.length; i++) {
      System.out.print(copia_interi[i]+ " ");
      if (copia_interi[i] != interi[i]) {
        counter++;
      }
    }
    System.out.println();

    if (counter > interi.length/2) {
      int[] res = Metodi.modificaElementi(interi, copia_interi);
      System.out.print("Res: ");
      for (int i=0; i<res.length; i++) {
        System.out.print(res[i]+ " ");
      }
    }
  }
}

class Metodi {
  public static void occorrenzaElementi(int[] par, int a, int b) {
    for (int i=0; i<par.length; i++) {
      if (par[i] < a) {
        par[i] = b;
      } else if (par[i] > b) {
        par[i] = a;
      }
    }
  }

  public static int[] modificaElementi(int[] par1, int[] par2) {
    int[] res = new int[par1.length*2];

    for (int i=0, par1_i = 0, par2_i = 0; i<res.length; i++) {
      if (i%2==0) {
        res[i] = par1[par1_i++];
      } else {
        res[i] = par2[par2_i++];
      }
    }

    return res;
  }
}
