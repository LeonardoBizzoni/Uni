public class EsameA {
    public static void main(String[] args) {
      if (args.length != 6) {
        System.out.println("Numero di parametri invalido.");
        System.exit(1);
      }

      int n = Integer.parseInt(args[5]);
      int[] interi = new int[5];
      int[] copia_interi = new int[5];

      for (int i=0; i<interi.length; i++) {
        interi[i] = Integer.parseInt(args[i]);
        copia_interi[i] = interi[i];
      }

      Metodi.modificaArray(copia_interi, n);

      System.out.print("Interi: ");
      for (int i=0; i<interi.length; i++) {
        System.out.print(interi[i]+ " ");
      }
      System.out.print("\nCopia: ");

      for (int i=0; i<copia_interi.length; i++) {
        System.out.print(copia_interi[i]+ " ");
      }
      System.out.println();

      switch(Metodi.valutaDifferenzaArray(interi, copia_interi)) {
        case -1:
          System.out.println("Errore");
          System.exit(1);
          break;

        case 0:
          for (int i=0; i<copia_interi.length; i++) {
            System.out.print(copia_interi[i]+ " ");
          }
          System.out.println();
          break;

        default:
          for (int i=0; i<interi.length; i++) {
            System.out.print(interi[i]+ " ");
          }
          System.out.println();
      }
    }
}

class Metodi {
  public static void modificaArray(int[] par, int a) {
    int sum = 0;
    int prod = 1;

    for (int i=0; i<par.length; i++) {
      sum+=par[i];
      prod*=par[i];
    }

    if (a>prod-sum && a<prod+sum) {
      for (int i=0; i<par.length; i++) {
        if (par[i]>a) {
            par[i] = prod+sum;
        }
      }
    } else {
      for (int i=0; i<par.length; i++) {
        if (par[i]<a) {
            par[i] = prod-sum;
        }
      }
    }
  }

  public static int valutaDifferenzaArray(int[] par1, int[] par2) {
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
