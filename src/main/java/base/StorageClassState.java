package base;

import org.lmdbjava.Env;
import static org.lmdbjava.Env.create;

import java.io.File;
import java.nio.ByteBuffer;

public class StorageClassState {
	private static final Env<ByteBuffer> lmdbEnv = create()
			.setMapSize(8589934592L)
			.setMaxDbs(64)
			.open(new File(".ODEstore"));
	public StorageClassState(){};
	public static Env<ByteBuffer> getLmdbEnv() {
		return lmdbEnv;
	}
}
