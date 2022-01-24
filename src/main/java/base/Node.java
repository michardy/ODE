package base;

import java.nio.ByteBuffer;

public interface Node {
	Fragment listNodes();
	Node[] getNodes(Fragment f);
	void createNode(Fragment f);
	void moveNode(Fragment f);
	void linkNode(Fragment f);
	void unlinkNode(Fragment f);
	ByteBuffer read(Fragment f);
	void write(ByteBuffer b);
}
