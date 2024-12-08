import java.io.FileWriter;
import java.io.IOException;
import java.io.PrintWriter;

public class LogManager {

    private static final LogManager instance = new LogManager();
    
    private static final String LOG_FILE = "system_logs.txt";
    
    private PrintWriter printWriter;

    private LogManager() {
        initializeLog();
    }

    public static LogManager getInstance() {
        return instance;
    }

    private void initializeLog() {
        try {
            FileWriter fileWriter = new FileWriter(LOG_FILE, true);
            printWriter = new PrintWriter(fileWriter);
            printWriter.println("Sistema de Log Inicializado");
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public void log(String message) {
        if (printWriter != null) {
            printWriter.println(message);
        }
        System.out.println("Log registrado: " + message);
    }

    public void close() {
        if (printWriter != null) {
            printWriter.close();
        }
    }

    public static void main(String[] args) {
        LogManager log1 = LogManager.getInstance();
        LogManager log2 = LogManager.getInstance();
        
        System.out.println(log1 == log2);

        log1.log("Evento 1: Sistema iniciado.");
        log2.log("Evento 2: Usuário autenticado.");

        log1.close();
    }
}
