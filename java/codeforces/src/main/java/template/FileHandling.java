package template;

import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;

public class FileHandling {
    public static void main(String[] args) {
        try {
            FileReader fr = new FileReader("data.in");
            FileWriter fw = new FileWriter("data.out");
            String str = "";
            int i;

            while ((i = fr.read()) != -1) str += (char) i;
            System.out.println(str);
            fw.write(str);
            fr.close();
            fw.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
