package runtime;

import java.util.concurrent.BlockingQueue;
import java.util.concurrent.LinkedBlockingQueue;

public class Thread {
	static final BlockingQueue<Packet> queue = new LinkedBlockingQueue<>();
}
