package nativestore;

import base.Fragment;
import base.Node;

import java.nio.ByteBuffer;

public class NativeNode implements Node {
	@Override
	public NativeFragment listNodes() {
		return null;
	}

	@Override
	public Node getNodes(Fragment f) {
		return null;
	}

	@Override
	public void createNode(Fragment f) {

	}

	@Override
	public void moveNode(Fragment f) {

	}

	@Override
	public void linkNode(Fragment f) {

	}

	@Override
	public void unlinkNode(Fragment f) {

	}

	@Override
	public ByteBuffer read(Fragment f) {
		return null;
	}

	@Override
	public void write(ByteBuffer b) {

	}
}
