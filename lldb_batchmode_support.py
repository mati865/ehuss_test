def breakpoint_callback(frame, bp_loc, dict):
    """This callback is registered with every breakpoint and makes sure that the
    frame containing the breakpoint location is selected """

    # HACK(eddyb) print a newline to avoid continuing an unfinished line.
    print("")
    print("Hit breakpoint " + str(bp_loc))

    # Select the frame and the thread containing it
    frame.thread.process.SetSelectedThread(frame.thread)
    frame.thread.SetSelectedFrame(frame.idx)

    # Returning True means that we actually want to stop at this breakpoint
    return True
