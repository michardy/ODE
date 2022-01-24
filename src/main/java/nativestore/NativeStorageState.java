package nativestore;

import base.StorageClassState;
import org.lmdbjava.Dbi;
import org.lmdbjava.DbiFlags;

import java.nio.ByteBuffer;

public class NativeStorageState extends StorageClassState {
	private static Dbi<ByteBuffer> nodeDbi = null;
	private static Dbi<ByteBuffer> blobDbi = null;
	NativeStorageState() {
		super();
		if (nodeDbi == null) {
			nodeDbi = super.getLmdbEnv().openDbi("NativestoreNodes", DbiFlags.MDB_CREATE);
		}
		if (blobDbi == null) {
			blobDbi = super.getLmdbEnv().openDbi("NativestoreBlobs", DbiFlags.MDB_CREATE);
		}
	}
}
