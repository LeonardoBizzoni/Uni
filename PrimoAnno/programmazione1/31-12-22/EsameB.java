public class EsameB {
    public static void main(String[] args) {
      if (args.length != 6) {
        System.out.println("Numero di parametri invalido.");
        System.exit(1);
      }

      char c = args[5].charAt(0);
      char[] caratteri = new char[5];
      char[] copia_caratteri = new char[5];

      for (int i=0; i<caratteri.length; i++) {
        caratteri[i] = args[i].charAt(0);
        copia_caratteri[i] = caratteri[i];
      }

      Metodi.modificaArray(copia_caratteri, c);

      System.out.print("Caratteri: ");
      for (int i=0; i<caratteri.length; i++) {
        System.out.print(caratteri[i]+ " ");
      }
      System.out.print("\nCopia: ");

      for (int i=0; i<copia_caratteri.length; i++) {
        System.out.print(copia_caratteri[i]+ " ");
      }
      System.out.println();

      switch(Metodi.valutaDifferenzaArray(caratteri, copia_caratteri)) {
        case -1:
          System.out.println("Errore");
          System.exit(1);
          break;

        case 0:
          for (int i=0; i<copia_caratteri.length; i++) {
            System.out.print(copia_caratteri[i]+ " ");
          }
          System.out.println();
          break;

        default:
          for (int i=0; i<caratteri.length; i++) {
            System.out.print(caratteri[i]+ " ");
          }
          System.out.println();
      }
    }
}

class Metodi {
  public static void modificaArray(char[] par, char c) {
    int counter = 0;

    for (int i=0; i<par.length; i++) {
      if (par[i] == c) {
        counter++;
      }
    }

    if (counter == 0) {
      for (int i=0; i<par.length; i++) {
        if (i%2==0) {
          par[i] = c;
        }
      }
    } else if (counter < par.length) {
      for (int i=0; i<par.length; i++) {
        if (i%2!=0) {
          par[i] = c;
        }
      }
    }
  }

  public static int valutaDifferenzaArray(char[] par1, char[] par2) {
    if (par1.length == par2.length) {
      for (int i=0; i<par1.length; i++) {
        if (par1[i] != par2[i]) {
            return 1;
        }
      }
      return 0;
    }
    return -1;
  }
}
