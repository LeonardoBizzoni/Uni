public class EsameD {
  public static void main(String[] args) {
    if (args.length != 1) {
      System.out.println("Numero di parametri invalido");
      System.exit(1);
    }

    String stringa = args[0];
    char[] pari, dispari;
    boolean changed = false;

    if (stringa.length()%2==0) {
      pari = new char[stringa.length()/2];
      dispari = new char[stringa.length()/2];
    } else {
      pari = new char[stringa.length()/2+1];
      dispari = new char[stringa.length()/2];
    }

    for (int i=0, pari_i=0, dispari_i=0; i<stringa.length(); i++) {
      if (i%2==0) {
        pari[pari_i++] = stringa.charAt(i);
      } else {
        dispari[dispari_i++] = stringa.charAt(i);
      }
    }

    char[] copia_dispari = new char[dispari.length];

    System.out.print("Pari pre modifica:");
    for (int i=0; i<pari.length; i++) {
      System.out.print(pari[i]+ " ");
    }
    System.out.print("\nDispari pre modifica:");
    for (int i=0; i<dispari.length; i++) {
      System.out.print(dispari[i]+ " ");
      copia_dispari[i] = dispari[i];
    }
    System.out.println();

    Metodi.modificaArray(pari, dispari);

    System.out.print("\nPari post modifica:");
    for (int i=0; i<pari.length; i++) {
      System.out.print(pari[i]+ " ");
    }
    System.out.print("\nDispari post modifica:");
    for (int i=0; i<dispari.length; i++) {
      System.out.print(dispari[i]+ " ");
      if (dispari[i] != copia_dispari[i]) {
        changed = true;
      }
    }

    if (changed) {
      String res = Metodi.unisciArray(pari, dispari);
      System.out.print("\nUnione pari dispari:"+res);
    }
  }
}

class Metodi {
  public static void modificaArray(char[] pari, char[] dispari) {
    if (pari.length < dispari.length) {
      for (int i=0; i<pari.length; i++) {
        switch(pari[i]) {
          case 'a':
            pari[i] = 'A';
            break;
          case 'e':
            pari[i] = 'E';
            break;
          case 'i':
            pari[i] = 'I';
            break;
          case 'o':
            pari[i] = 'O';
            break;
          case 'u':
            pari[i] = 'U';
            break;
        }
      }
    } else if (pari.length > dispari.length) {
      for (int i=0; i<dispari.length; i++) {
        switch(dispari[i]) {
          case 'a':
          case 'e':
          case 'i':
          case 'o':
          case 'u':
            break;
          default:
            dispari[i] = (char)((int)dispari[i]-32);
        }
      }
    }
  }

  public static String unisciArray(char[] par1, char[] par2) {
    String res = "";

    for (int i=0; i<par1.length; i++) {
      res += par1[i];
    }
    for (int i=0; i<par2.length; i++) {
      res += par2[i];
    }

    return res;
  }
}
