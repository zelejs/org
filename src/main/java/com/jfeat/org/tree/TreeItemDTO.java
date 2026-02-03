package com.jfeat.org.tree;


import java.util.List;

/**
 * @author : lvsongxin
 */
public class TreeItemDTO<T> {
   private Long id;
   private Long pid;
   private List<T> children;


   public Long getId() {
      return id;
   }

   public void setId(Long id) {
      this.id = id;
   }

   public Long getPid() {
      return pid;
   }

   public void setPid(Long pid) {
      this.pid = pid;
   }

   public List<T> getChildren() {
      return children;
   }

   public void setChildren(List<T> children) {
      this.children = children;
   }
}
