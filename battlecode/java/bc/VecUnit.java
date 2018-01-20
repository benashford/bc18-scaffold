/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (http://www.swig.org).
 * Version 3.0.10
 *
 * Do not make changes to this file unless you know what you are doing--modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package bc;

public class VecUnit {
  private transient long swigCPtr;
  protected transient boolean swigCMemOwn;

  protected VecUnit(long cPtr, boolean cMemoryOwn) {
    swigCMemOwn = cMemoryOwn;
    swigCPtr = cPtr;
  }

  protected static long getCPtr(VecUnit obj) {
    return (obj == null) ? 0 : obj.swigCPtr;
  }

  protected void finalize() {
    delete();
  }

  public synchronized void delete() {
    if (swigCPtr != 0) {
      if (swigCMemOwn) {
        swigCMemOwn = false;
        bcJNI.delete_VecUnit(swigCPtr);
      }
      swigCPtr = 0;
    }
  }

  public VecUnit() {
    this(bcJNI.new_VecUnit(), true);
  }

  public String toString() {
    return bcJNI.VecUnit_toString(swigCPtr, this);
  }

  public VecUnit clone() {
    long cPtr = bcJNI.VecUnit_clone(swigCPtr, this);
    return (cPtr == 0) ? null : new VecUnit(cPtr, true);
  }

  public long size() {
    return bcJNI.VecUnit_size(swigCPtr, this);
  }

  public Unit get(long index) {
    long cPtr = bcJNI.VecUnit_get(swigCPtr, this, index);
    return (cPtr == 0) ? null : new Unit(cPtr, true);
  }

}
