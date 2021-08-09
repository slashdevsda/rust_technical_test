
import os

steps = 0

def can_permute(idx, seq):
    # import pdb; pdb.set_trace()

    # assert idx == len(seq) or idx + 1 < len(seq)
    if idx == len(seq) - 1:
        return True

    # last star constrain
    if seq[idx + 1] == '1' and idx + 2 == len(seq):
        return True

    if seq[idx + 1] == '1' and \
        all(i == '0' for i in seq[idx + 2:]):
        return True

    return False


def permute_to(ch, idx, seq):
    global steps
    
    # import pdb; pdb.set_trace()
        
    if seq[idx] == ch:
        return seq

    if not can_permute(idx, seq):
        if seq[idx + 1] != '1':
            # import pdb;pdb.set_trace()
            seq = permute_to('1', idx + 1, seq)
        
        if not all(i == '0' for i in seq[idx + 1:]):
            # import pdb;pdb.set_trace()
            for i in range(idx + 2, len(seq)):
                seq = permute_to('0', i, seq)

    if can_permute(idx, seq):
        seq[idx] = ch 
        steps += 1
        print("[%s] permute! at index %s" %(steps, '-'.join(seq)))
    return seq


    # if idx == len(seq):
    #     steps += 1
    #     seq[idx] = ch
    #     return seq

    # constains


# def permute(idx, seq):
#     while seq[idx + 1] != 1 and \
#         all(i != '0' for i in seq[idx + 2:]):
#         # tail = seq[idx + 2:]
        
#         permute_to('1', idx, seq)


# def change_char(idx, start, target_char):
#     if can_permute(idx, start):
#         start[idx] = target_char
#         return start
#     else:
#         # print(idx, start, target_char)
#         # change_char(idx + 1, start, 1)
#         permute(idx, start)

def process(start, target):
    res = list(start)
    for i in range(len(res)):
        if (res[i] == target[i]):
            continue
        else:
            print("trying to permut index ", i, " res = ", res, " target = ",target)
            res = permute_to(target[i], i, res)
    print("res = ", res)
    #for idx, c in enumerate(start):
    #    if c == target[idx]:
    #        continue
    #    else:
    #        #print(f"[{start}] trying to permute {c} at position {idx} to {target[idx]}")
    #        start = permute_to(target[idx], idx, start)

process("11001001000", "10000110011")
