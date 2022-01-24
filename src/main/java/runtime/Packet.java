package runtime;

import base.Fragment;
import base.Node;

import java.util.ArrayList;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;

public class Packet {
	private Node node;
	private Message message;
	private ArrayList<Fragment> from;
	private ArrayList<Fragment> to;

	Node[] getNextNodes() {
		return node.getNodes(to.remove(0));
	}

	Packet createForwarded(Node base) throws CloneNotSupportedException {
		Packet newP = (Packet)this.clone();
		newP.node = base;
		return newP;
	}

	public void process(BlockingQueue<Packet> queue) throws CloneNotSupportedException, InterruptedException {
		Node[] nodes = getNextNodes();
		if (nodes.length == 0) {

		} else {
			for (Node n: nodes) {
				queue.offer(createForwarded(n), 1000, TimeUnit.MILLISECONDS);
			}
		}
	}
}
