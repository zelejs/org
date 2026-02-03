package com.jfeat.org.tree;

import com.jfeat.crud.base.exception.BusinessException;

import java.util.*;

/**
 * @author : lvsongxin
 * @date :
 */
public class TreeUtls {

    public static <T extends TreeItemDTO> T buildTree(List<T> list, Long top) {
        Map<Long, T> itemMap = new HashMap<>();
        for(T item: list) {
            item.setChildren(new ArrayList<T>());
            itemMap.put(item.getId(),item);
        }
        for(T item : list) {
            if(itemMap.containsKey(item.getPid())) {
                List<T> items = itemMap.get(item.getPid()).getChildren();
                items.add(item);
            }
        }
        Optional<T> any = list.stream().filter(s -> s.getId().equals(top)).findAny();
        if(any.isEmpty()) {
            throw new BusinessException(-1, "找不到顶级节点");
        }
        return any.get();
    }
}
